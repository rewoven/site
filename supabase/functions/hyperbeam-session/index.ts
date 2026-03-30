import "jsr:@supabase/functions-js/edge-runtime.d.ts";

const HYPERBEAM_API_KEY = Deno.env.get("HYPERBEAM_API_KEY")!;
const EXTENSION_URL = "https://github.com/rewoven/extension/releases/latest/download/rewoven-shopping-lens.zip";

Deno.serve(async (req) => {
  // CORS
  if (req.method === "OPTIONS") {
    return new Response(null, {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "POST, OPTIONS",
        "Access-Control-Allow-Headers": "Content-Type, apikey, Authorization",
      },
    });
  }

  try {
    const { start_url, with_extension } = await req.json();

    let body: BodyInit;
    let headers: Record<string, string> = {
      Authorization: `Bearer ${HYPERBEAM_API_KEY}`,
    };

    const sessionConfig: Record<string, unknown> = {
      start_url: start_url || "https://www.zara.com",
      kiosk: false,
      dark: true,
      adblock: true,
      width: 1280,
      height: 720,
      fps: 30,
      timeout: {
        absolute: 300,
        inactive: 120,
        offline: 30,
      },
      region: "NA",
    };

    if (with_extension !== false) {
      // Try to download and attach extension
      try {
        const extResponse = await fetch(EXTENSION_URL, { redirect: "follow" });
        if (extResponse.ok) {
          const extBlob = await extResponse.blob();
          const formData = new FormData();
          formData.append("ex", extBlob, "extension.zip");
          sessionConfig.extension = { field: "ex" };
          formData.append("body", JSON.stringify(sessionConfig));
          body = formData;
        } else {
          // Extension download failed, proceed without it
          headers["Content-Type"] = "application/json";
          body = JSON.stringify(sessionConfig);
        }
      } catch {
        // Extension download failed, proceed without it
        headers["Content-Type"] = "application/json";
        body = JSON.stringify(sessionConfig);
      }
    } else {
      headers["Content-Type"] = "application/json";
      body = JSON.stringify(sessionConfig);
    }

    const response = await fetch("https://engine.hyperbeam.com/v0/vm", {
      method: "POST",
      headers,
      body,
    });

    if (!response.ok) {
      const error = await response.text();
      return new Response(JSON.stringify({ error, status: response.status }), {
        status: 502,
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Origin": "*",
        },
      });
    }

    const data = await response.json();

    return new Response(
      JSON.stringify({
        embed_url: data.embed_url,
        session_id: data.session_id,
      }),
      {
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Origin": "*",
        },
      }
    );
  } catch (err) {
    return new Response(
      JSON.stringify({ error: (err as Error).message, stack: (err as Error).stack }),
      {
        status: 500,
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Origin": "*",
        },
      }
    );
  }
});
