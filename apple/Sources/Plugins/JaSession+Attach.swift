//
//  JaSession+Attach.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import Foundation
import JanusGateway

extension JaSession {
    /// Attach to echotest plugin
    ///
    /// - Returns: An echotest plugin handle
    public func attachEchoTest(timeout: TimeInterval) async throws -> JaEchoTestHandle {
        let handle = try await lower.attachEchoTest(timeout: timeout)
        return JaEchoTestHandle(handle: handle)
    }
}
