package com.ghamza.janus.core

import com.ghamza.janus.bindings.Connection
import com.ghamza.janus.bindings.rawJanusConnect

class JaConnection(val connection: Connection) {
    companion object {
        suspend fun connect(config: JaConfig): JaConnection {
            val connection = rawJanusConnect(config.lower)
            return JaConnection(connection = connection)
        }
    }
}
