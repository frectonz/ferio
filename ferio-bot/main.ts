import { serve } from "https://deno.land/std@0.156.0/http/server.ts";
import { webhookCallback } from "https://deno.land/x/grammy@v1.11.0/mod.ts";
import bot from "./bot.ts";

const handleUpdate = webhookCallback(bot, "std/http");

serve(async (req) => {
  if (req.method == "POST") {
    console.log("Received update");
    const url = new URL(req.url);
    console.log("URL", url);
    if (url.pathname.slice(1) == bot.token) {
      console.log("Handling update");
      try {
        return await handleUpdate(req);
      } catch (err) {
        console.error(err);
      }
    }
  }
  return new Response();
});
