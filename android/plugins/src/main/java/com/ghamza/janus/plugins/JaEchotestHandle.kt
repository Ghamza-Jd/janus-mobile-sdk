package com.ghamza.janus.plugins

import com.ghamza.janus.bindings.EchotestHandle
import com.ghamza.janus.bindings.EchotestHandleCallback
import com.ghamza.janus.bindings.Jsep
import kotlinx.coroutines.channels.ProducerScope
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.callbackFlow
import java.time.Duration

class JaEchotestHandle(val handle: EchotestHandle): EchotestHandleCallback {
    private var events: ProducerScope<JaEchotestEvent>? = null

    fun stream() = callbackFlow {
        events = this
        handle.startEventLoop(this@JaEchotestHandle)
        awaitClose { }
    }

    suspend fun start(audio: Boolean = false, video: Boolean = false, bitrate: UInt? = null) {
        handle.start(audio = audio, video = video, bitrate = bitrate)
    }

    suspend fun start(
        audio: Boolean = false,
        video: Boolean = false,
        bitrate: UInt? = null,
        jsep: Jsep,
        timeout: Duration
    ) {
        handle.startWithJsep(audio = audio, video = video, bitrate = bitrate, jsep = jsep, timeout = timeout)
    }

    override fun onResult(echotest: String, result: String) {
        events?.trySend(JaEchotestEvent.Result(echotest = echotest, result = result))
    }

    override fun onResultWithJsep(echotest: String, result: String, jsep: Jsep) {
        events?.trySend(JaEchotestEvent.ResultWithJsep(echotest = echotest, result = result, jsep = jsep))
    }
}

sealed interface JaEchotestEvent {
    data class Result(val echotest: String, val result: String): JaEchotestEvent
    data class ResultWithJsep(val echotest: String, val result: String, val jsep: Jsep): JaEchotestEvent
}
