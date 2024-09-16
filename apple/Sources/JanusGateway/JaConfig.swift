//
//  JaConfig.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import UniFFI

/// A struct describing a janus server (e.g. url, secret).
public struct JaConfig {
    let url: String
    let capacity: UInt16
    let namespace: String?
    let apisecret: String?

    var lower: Config {
        Config(
            url: url,
            capacity: capacity,
            apisecret: apisecret,
            namespace: namespace
        )
    }

    /// - Parameters:
    ///     - url: The URL to reach this server API
    ///     - capacity: Buffer capacity
    ///     - namespace: The connection namespace, default ot "janus"
    ///     - apisecrent: The API secret for this server
    public init(
        url: String,
        capacity: UInt16,
        namespace: String? = nil,
        apisecret: String? = nil
    ) {
        self.url = url
        self.capacity = capacity
        self.namespace = namespace
        self.apisecret = apisecret
    }
}
