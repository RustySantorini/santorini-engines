from surrealdb import Surreal

async def main():
    async with Surreal("wss://db-deploy.fly.dev/rpc") as db:
        await db.signin({"user": "root", "pass": "root"})
        await db.use("db", "game")

if __name__ == "__main__":
    import asyncio

    asyncio.run(main())