// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Jarust",
    platforms: [
        .iOS(.v15)
    ],
    products: [
        .library(name: "Jarust", targets: ["Jarust"]),
    ],
    targets: [
        .target(name: "Jarust", dependencies: ["JarustNative"]),
        .binaryTarget(
            name: "JarustNative",
            url: "https://github.com/Ghamza-Jd/jarust-mobile-sdk/releases/download/v0.1.0/JarustNative.zip", 
            checksum: "e811e5cb123131d3d4ccc2836834e276424a6e86d8e1402f57d8c04eb6b46a1d"
        ),
        .testTarget(name: "JarustTests", dependencies: ["Jarust"]),
    ]
)
