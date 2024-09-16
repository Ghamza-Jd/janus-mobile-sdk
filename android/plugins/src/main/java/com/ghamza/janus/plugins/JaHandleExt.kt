package com.ghamza.janus.plugins

import com.ghamza.janus.core.JaSession
import java.time.Duration

suspend fun JaSession.attachEchoTest(timeout: Duration): JaEchotestHandle {
    val handle = this.lower.attachEchoTest(timeout)
    return JaEchotestHandle(handle = handle)
}
