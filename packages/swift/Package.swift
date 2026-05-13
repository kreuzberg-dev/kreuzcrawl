// swift-tools-version: 6.0
import PackageDescription
import Foundation

// NOTE: Run `cargo build -p kreuzcrawl-swift` before `swift build`.
// The build step generates Swift + C bridge sources; copy them into Sources/RustBridge
// and Sources/RustBridgeC before building. See BUILDING.md for the full workflow.

// Derive absolute Cargo target paths. SwiftPM evaluates Package.swift with CWD = package dir
// (packages/swift), so two levels up is the kreuzcrawl repo root.
let repoRoot = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
    .deletingLastPathComponent()   // packages/swift → packages
    .deletingLastPathComponent()   // packages → kreuzcrawl root
    .path
let targetDebug = repoRoot + "/target/debug"
let targetRelease = repoRoot + "/target/release"

let package = Package(
    name: "Kreuzcrawl",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    products: [
        .library(name: "Kreuzcrawl", targets: ["Kreuzcrawl"]),
    ],
    targets: [
        // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
        // to access C types (RustStr, etc.) produced by swift-bridge.
        // publicHeadersPath: "." exposes RustBridgeC.h to dependents.
        .target(
            name: "RustBridgeC",
            path: "Sources/RustBridgeC",
            publicHeadersPath: "."
        ),
        // RustBridge: Swift wrapper around the Rust static library.
        // Depends on RustBridgeC so the generated Swift files can use the C types.
        // linkerSettings wire the Rust staticlib (libkreuzcrawl_swift.a) produced by
        // `cargo build -p kreuzcrawl-swift` so `swift build` / `swift test` can resolve
        // the `__swift_bridge__$*` C symbols. Both target/release and target/debug are
        // searched so either cargo profile works.
        .target(
            name: "RustBridge",
            dependencies: ["RustBridgeC"],
            path: "Sources/RustBridge",
            linkerSettings: [
                // -L paths must precede -l so the linker finds the static lib before
                // attempting to resolve -lkreuzcrawl_swift. All flags in one unsafeFlags
                // block preserves declaration order.
                .unsafeFlags([
                    "-L\(targetDebug)",
                    "-L\(targetRelease)",
                    "-lkreuzcrawl_swift",
                ]),
                .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
                .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
                .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
            ]
        ),
        .target(name: "Kreuzcrawl", dependencies: ["RustBridge"], path: "Sources/Kreuzcrawl"),
        .testTarget(name: "KreuzcrawlTests", dependencies: ["Kreuzcrawl"], path: "Tests/KreuzcrawlTests"),
    ]
)
