//
//  JaHandle.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import Foundation
import UniFFI

/// General purpose plugin handle
public final class JaHandle {
    let handle: Handle
    public var delegate: JaHandleDelegate?
    private var continuation: AsyncStream<String>.Continuation?

    /// Get an async stream of incoming Janus events for this handle
    ///
    /// - Returns: An async stream of incoming events
    public var events: AsyncStream<String> {
        get async {
            await handle.startEventLoop(cb: self)

            return AsyncStream { continuation in
                self.continuation = continuation
            }
        }
    }

    init(handle: Handle) {
        self.handle = handle
    }

    /// Sends a message without waiting for any response or acknowledgment
    ///
    /// - Parameters:
    ///     - msg: Message to be sent
    public func fireAndForget(msg: String) async throws {
        try await handle.fireAndForget(msg: msg)
    }

    /// Sends a message without waiting for any response or acknowledgment
    ///
    /// - Parameters:
    ///     - msg: Message to be sent
    ///     - jsep: JavaScript Session Establishment Protocol
    public func fireAndForget(msg: String, jsep: Jsep) async throws {
        try await handle.fireAndForgetWithJsep(msg: msg, jsep: jsep)
    }

    /// Sends a message and waits until the server acknowledges or timeout
    ///
    /// - Parameters:
    ///     - msg: Message to be sent
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    public func sendWaitonAck(
        msg: String, timeout: TimeInterval
    ) async throws {
        try await handle.sendWaitonAck(msg: msg, timeout: timeout)
    }

    /// Sends a message and waits until the server returns a response or timeout
    ///
    /// - Parameters:
    ///     - msg: Message to be sent
    ///     - timeout: The maximum amount of time to wait on a response before we consider the
    ///     request as failed or times out.
    public func sendWaitonResult(
        msg: String,
        timeout: TimeInterval
    ) async throws -> String {
        try await handle.sendWaitonResult(msg: msg, timeout: timeout)
    }
}

extension JaHandle: HandleCallback {
    public func onEvent(event: String) {
        delegate?.didReceive(event: event)
        continuation?.yield(event)
    }
}
