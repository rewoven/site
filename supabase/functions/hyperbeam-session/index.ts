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
    const { start_url } = await req.json();

    // Download extension zip
    const extResponse = await fetch(EXTENSION_URL);
    if (!extResponse.ok) {
      throw new Error("Failed to download extension");
    }
    const extBlob = await extResponse.blob();

    // Create multipart form data
    const formData = new FormData();
    formData.append("ex", extBlob, "extension.zip");
    formData.append(
      "body",
      JSON.stringify({
        start_url: start_url || "https://www.zara.com",
        kiosk: false,
        dark: true,
        adblock: true,
        width: 1280,
        height: 720,
        fps: 30,
        timeout: {
          absolute: 300, // 5 minute max session
          inactive: 120, // 2 min idle timeout
          offline: 30,
        },
        extension: {
          field: "ex",
        },
        region: "NA",
      })
    );

    const response = await fetch("https://engine.hyperbeam.com/v0/vm", {
      method: "POST",
      headers: {
        Authorization: `Bearer ${HYPERBEAM_API_KEY}`,
      },
      body: formData,
    });

    if (!response.ok) {
      const error = await response.text();
      return new Response(JSON.stringify({ error }), {
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
      JSON.stringify({ error: (err as Error).message }),
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
