package com.ghamza.janus.core

import com.ghamza.janus.bindings.Config

data class JaConfig(
    val url: String, val capacity: UShort, val namespace: String?, val apisecret: String?
) {
    val lower: Config
        get() = Config(url, capacity, namespace, apisecret)
}
