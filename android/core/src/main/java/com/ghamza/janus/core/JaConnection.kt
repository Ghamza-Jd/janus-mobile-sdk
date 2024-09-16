package com.ghamza.janus.core

import com.ghamza.janus.bindings.Connection
import com.ghamza.janus.bindings.rawJanusConnect
import java.time.Duration

class JaConnection(val connection: Connection) {
    companion object {
        suspend fun connect(config: JaConfig): JaConnection {
            val connection = rawJanusConnect(config.lower)
            return JaConnection(connection = connection)
        }
    }

    suspend fun createSession(kaInterval: UInt, timeout: Duration): JaSession {
        val session = connection.createSession(kaInterval = kaInterval, timeout = timeout)
        return JaSession(session = session)
    }
}
