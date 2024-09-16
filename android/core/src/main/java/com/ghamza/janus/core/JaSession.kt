package com.ghamza.janus.core

import com.ghamza.janus.bindings.Session
import java.time.Duration

class JaSession(val session: Session) {
    val lower: Session
        get() = session

    suspend fun attach(pluginId: String, timeout: Duration): JaHandle {
        val handle = session.attach(pluginId = pluginId, timeout = timeout)
        return JaHandle(handle = handle)
    }
}
