// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v14),
    ],
    dependencies: [
        .package(path: "../../packages/swift"),
    ],
    targets: [
        .testTarget(
            name: "KreuzcrawlTests",
            dependencies: [.product(name: "Kreuzcrawl", package: "swift")]
        ),
    ]
)
