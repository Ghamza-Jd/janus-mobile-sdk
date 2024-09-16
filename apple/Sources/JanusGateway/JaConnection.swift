//
//  JaConnection.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import UniFFI

/// Connection with a Janus server
public struct JaConnection {
    let connection: Connection

    private init(connection: Connection) {
        self.connection = connection
    }

    /// Connect using the provided configuration.
    ///
    /// - Parameters:
    ///     - config: Janus connection configuration
    /// - Returns: A connection with janus server
    public static func connect(config: JaConfig) async throws -> Self {
        let connection = try await rawJarustConnect(config: config.lower)
        return JaConnection(connection: connection)
    }

    /// Create a client-server session
    ///
    /// - Parameters:
    ///     - kaInterval: The time interval (seconds) for session keep-alive requests
    /// - Returns: The newly created session
    public func createSession(kaInterval: UInt32) async throws -> JaSession {
        let session = try await connection.createSession(kaInterval: kaInterval)
        return JaSession(session: session)
    }
}
