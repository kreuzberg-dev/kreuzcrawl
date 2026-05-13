import { createEngine, scrape } from "kreuzcrawl";

(async () => {
  const engine = createEngine(null);
  const url = `${process.env.MOCK_SERVER_URL}/fixtures/links_anchor_fragment`;
  const result = await scrape(engine, url);
  console.log("links[0]:", result.links[0]);
  console.log("linkType:", result.links[0].linkType);
  console.log("linkType type:", typeof result.links[0].linkType);
  console.log("linkType value:", JSON.stringify(result.links[0].linkType));
})();
