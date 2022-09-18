import { Bot } from "https://deno.land/x/grammy@v1.11.0/mod.ts";

const TOKEN = Deno.env.get("TOKEN");

if (!TOKEN) {
  console.error("Please set the TOKEN environment variable.");
  Deno.exit();
}

const bot = new Bot(TOKEN);

bot.api.setMyCommands([
  {
    command: "start",
    description: "Show introduction message",
  },
  {
    command: "help",
    description: "Show help message",
  },
  {
    command: "today",
    description: "Get today's holidays and observances",
  },
]);

bot.command(
  "start",
  (ctx) =>
    ctx.reply(
      "This is a bot that helps you get the holidays for a given date (/date) or today (/today). It sends you then names of the holidays with their wikipedia links and images of the holiday.",
    ),
);

bot.command(
  "help",
  (ctx) =>
    ctx.reply(
      "/start - show introduction message\n" +
        "/today - get today's holidays\n" +
        "/date - get holidays for a given date\n" +
        "/help - show this message",
    ),
);

interface Data {
  date: string;
  data: {
    name: string;
    wikipedia_url: string;
    image_url?: string;
    greeting: string;
  }[];
}

const SERVER_URL = Deno.env.get("SERVER_URL") || "http://0.0.0.0:3000";

bot.command("today", async (ctx) => {
  try {
    const data = await fetch(SERVER_URL);
    const json = await data.json() as Data;

    for (const holiday of json.data) {
      const message =
        `<a href="${holiday.wikipedia_url}">${holiday.greeting}</a>`;

      if (holiday.image_url) {
        const isSvg = holiday.image_url.endsWith(".svg");
        if (!isSvg) {
          await ctx.replyWithPhoto(holiday.image_url, {
            caption: message,
            parse_mode: "HTML",
          });
        } else {
          await ctx.reply(message, { parse_mode: "HTML" });
        }
      } else {
        await ctx.reply(message, { parse_mode: "HTML" });
      }
    }
  } catch {
    ctx.reply(
      "I wasn't able to get today's holidays. If it was an error on my side it will be fixed. Try again after some time.",
    );
  }
});

bot.catch((err) => console.error({ ...err }));

export default bot;
