import { Bot, Context } from "https://deno.land/x/grammy@v1.11.0/mod.ts";

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
  {
    command: "date",
    description: "Get holidays and observances for a specific date",
  },
]);

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

bot.command(
  "start",
  (ctx) =>
    ctx.reply(
      "This is a bot that helps you get the holidays for a given date (/date) or today (/today). It sends you then names of the holidays with their wikipedia links and images of the holiday.",
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

bot.command(
  "today",
  handleError(async (ctx) => {
    const data = await fetch(SERVER_URL);
    const json = await data.json() as Data;
    await sendHolidays(ctx, json);
  }),
);

bot.command(
  "date",
  handleError(async (ctx) => {
    let date = ctx.match;
    if (typeof date === "string") {
      date = date.trim().replace(/\s+/g, "_");
      const data = await fetch(`${SERVER_URL}/?date=${date}`);
      const json = await data.json() as Data;
      await sendHolidays(ctx, json);
    } else {
      await ctx.reply(
        "Please provide a date like this <pre>/date March 4</pre>",
        {
          parse_mode: "HTML",
        },
      );
    }
  }),
);

async function sendHolidays(ctx: Context, json: Data) {
  for (const holiday of json.data) {
    const message =
      `<a href="${holiday.wikipedia_url}">${holiday.greeting}</a>`;

    if (holiday.image_url) {
      try {
        await ctx.replyWithPhoto(holiday.image_url, {
          caption: message,
          parse_mode: "HTML",
        });
      } catch (error) {
        console.log("error sending image: ", holiday.image_url, error);
        await ctx.reply(message, { parse_mode: "HTML" });
      }
    } else {
      await ctx.reply(message, { parse_mode: "HTML" });
    }
  }
}

function handleError(fn: (ctx: Context) => Promise<void>) {
  return async (ctx: Context) => {
    try {
      await fn(ctx);
    } catch (error) {
      console.error(error)
      await ctx.reply(
        "I wasn't able to get today's holidays. If it was an error on my side it will be fixed. Try again after some time.",
      );
    }
  };
}

bot.catch((err) => console.error({ ...err }));

export default bot;
