```dart title="Dart"
import 'package:kreuzcrawl/kreuzcrawl.dart';
import 'package:kreuzcrawl/src/kreuzcrawl_bridge_generated/frb_generated.dart'
    show RustLib;

Future<void> main() async {
  await RustLib.init();

  // Simplest case: scrape a single page with default settings.
  final engine = await KreuzcrawlBridge.createEngine();
  final result = await KreuzcrawlBridge.scrape(engine, 'https://example.com/');
  print('Title: ${result.metadata.title ?? ''}');
  print('Status: ${result.statusCode}');
  print('Links found: ${result.links.length}');

  // Crawl from a seed URL, limited to one hop and a handful of pages.
  final crawlConfig = await createCrawlConfigFromJson(
    json: r'{"max_depth":1,"max_pages":5}',
  );
  final crawlEngine = await KreuzcrawlBridge.createEngine(config: crawlConfig);
  final crawlResult = await KreuzcrawlBridge.crawl(
    crawlEngine,
    'https://en.wikipedia.org/wiki/Web_scraping',
  );
  print('Pages crawled: ${crawlResult.pages.length}');
}
```
