//
//  JaEchoTestHandleDelegate.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import UniFFI

public protocol JaEchoTestHandleDelegate {
    func didReceive(echotest: String, result: String)
    func didRecieve(echotest: String, result: String, jsep: Jsep)
}
