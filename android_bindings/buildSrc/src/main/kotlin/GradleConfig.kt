object GradleConfigs {
    const val compileSdk = 34
    const val minSdk = 24

    // Different ndk version isn't working
    // lower is complaining about libunwind
    // higher can't compile openssl
    const val ndkVersion = "23.1.7779620"
}
