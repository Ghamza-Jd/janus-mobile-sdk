package com.ghamza.janus.core

import com.ghamza.janus.bindings.Handle
import com.ghamza.janus.bindings.HandleCallback
import com.ghamza.janus.bindings.Jsep
import kotlinx.coroutines.channels.ProducerScope
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.callbackFlow
import java.time.Duration

class JaHandle(val handle: Handle): HandleCallback {
    private var events: ProducerScope<String>? = null

    fun stream() = callbackFlow {
        events = this
        handle.startEventLoop(this@JaHandle)
        awaitClose { }
    }

    suspend fun fireAndForget(msg: String) {
        handle.fireAndForget(msg)
    }

    suspend fun fireAndForget(msg: String, jsep: Jsep) {
        handle.fireAndForgetWithJsep(msg, jsep)
    }

    suspend fun sendWaitOnAck(msg: String, timeout: Duration) {
        handle.sendWaitonAck(msg, timeout)
    }

    suspend fun sendWaitOnResult(msg: String, timeout: Duration): String {
        return handle.sendWaitonResult(msg, timeout)
    }

    override fun onEvent(event: String) {
        events?.trySend(event)
    }
}
