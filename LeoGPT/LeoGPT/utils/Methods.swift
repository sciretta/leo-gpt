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

func createChat(userId:String, chatName:String) async throws ->Bool {
    guard let url = URL(string: "http://localhost:3000/new_chat") else { return false }

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
        print("Status code:", httpResponse)
        
        if httpResponse.statusCode == 200 {
            let decoder = JSONDecoder()
            let chat = try decoder.decode(Chat.self, from: data)
            print(chat)
            return true
        }
    }
    
    return false
}
