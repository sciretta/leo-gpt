//
//  Methods.swift
//  LeoGPT
//
//  Created by Leonardo on 19/4/26.
//
import SwiftUI

func fetchMessages(chatId: String) async throws -> [Message] {
    guard let url = URL(string: "http://localhost:3000/messages") else { return [] }

    // Build the JSON body to mirror the curl payload
    let payload: [String: Any] = [
        "page": 1,
        "per_page": 5,
        "data": [
            "chat_id": chatId
        ]
    ]

    let bodyData = try JSONSerialization.data(withJSONObject: payload, options: [])

    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.httpBody = bodyData

    let (data, response) = try await URLSession.shared.data(for: request)

    if let httpResponse = response as? HTTPURLResponse {
        print("Status code:", httpResponse.statusCode)
    }

    let decoder = JSONDecoder()
    // The keys are snake_case and custom, but we've mapped them via CodingKeys
    let messages = try decoder.decode([Message].self, from: data)
    return messages
}



func fetchChats(username: String) async throws -> [Chat] {
    guard let url = URL(string: "http://localhost:3000/chats") else { return [] }

    // Build the JSON body to mirror the curl payload
    let payload: [String: Any] = [
        "page": 1,
        "per_page": 5,
        "data": [
            "username": username
        ]
    ]

    let bodyData = try JSONSerialization.data(withJSONObject: payload, options: [])

    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.httpBody = bodyData

    let (data, response) = try await URLSession.shared.data(for: request)

    if let httpResponse = response as? HTTPURLResponse {
        print("Status code:", httpResponse.statusCode)
    }

    let decoder = JSONDecoder()
    // The keys are snake_case and custom, but we've mapped them via CodingKeys
    let chats = try decoder.decode([Chat].self, from: data)
    return chats
}

func createChat(userId:String, chatName:String) async throws -> String {
    guard let url = URL(string: "http://localhost:3000/new_chat") else { return "" }

    // Build the JSON body to mirror the curl payload
    let payload: [String: Any] = [
        "chat_name": chatName,
        "user_id": userId
    ]

    let bodyData = try JSONSerialization.data(withJSONObject: payload, options: [])
    

    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.httpBody = bodyData

    let (data, response) = try await URLSession.shared.data(for: request)

    if let httpResponse = response as? HTTPURLResponse {
        print("Status code:", httpResponse.statusCode)
        
        if httpResponse.statusCode == 200 {
            let decoder = JSONDecoder()
            let chat = try decoder.decode(Chat.self, from: data)
            print(chat)
            return chat.id.oid
        }
    }
    
    return ""
}

final class SSEHandler: NSObject, URLSessionDataDelegate {
    private let onUpdate: (String) -> Void
    private var buffer = Data()
    
    init(onUpdate: @escaping (String) -> Void) {
        self.onUpdate = onUpdate
    }
    
    func urlSession(_ session: URLSession, dataTask: URLSessionDataTask, didReceive data: Data) {
        buffer.append(data)
        
        // Intenta convertir el buffer a String y buscar eventos completos (separados por \n\n)
        if let text = String(data: buffer, encoding: .utf8) {
            let events = text.components(separatedBy: "\n\n")
            // Procesa todos los eventos completos menos el último (que puede estar incompleto)
            for event in events.dropLast() {
                onUpdate(event)
            }
            // Guarda el fragmento incompleto para el próximo chunk
            if let last = events.last {
                buffer = Data(last.utf8)
            } else {
                buffer.removeAll(keepingCapacity: true)
            }
        }
    }
}

func createMessage(body: String, chatId: String, onUpdate: @escaping (String) -> Void) {
    let payload: [String: Any] = [
        "body": body,
        "chat_id": chatId
    ]

    guard let postData = try? JSONSerialization.data(withJSONObject: payload, options: []) else {
        onUpdate("Failed to serialize payload")
        return
    }

    guard let url = URL(string: "http://localhost:3000/new_message") else {
        onUpdate("Bad URL")
        return
    }

    var request = URLRequest(url: url, timeoutInterval: 0) // 0 para que no cierre por timeout
    request.addValue("application/json", forHTTPHeaderField: "Content-Type")
    request.httpMethod = "POST"
    request.httpBody = postData

    let handler = SSEHandler(onUpdate: onUpdate)
    let session = URLSession(configuration: .default, delegate: handler, delegateQueue: nil)
    let task = session.dataTask(with: request)
    task.resume()
}

