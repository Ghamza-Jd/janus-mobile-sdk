package com.ghamza.janus.core

import com.ghamza.janus.bindings.rawInitLogger

class JanusLogger {
    companion object {
        fun initialize() {
            rawInitLogger()
        }
    }
}
