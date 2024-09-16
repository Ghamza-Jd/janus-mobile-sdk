package com.ghamza.janus.core

import com.ghamza.janus.bindings.Session

class JaSession(val session: Session) {
    val lower: Session
        get() = session
}
