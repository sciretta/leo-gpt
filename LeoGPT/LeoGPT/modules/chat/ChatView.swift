//
//  ChatView.swift
//  LeoGPT
//
//  Created by Leonardo on 23/11/25.
//

import SwiftUI

struct ChatMessage:Identifiable{
    let id = UUID()
    let text:String
    let isUser:Bool
}

struct ChatView: View {
    @State private var messages: [ChatMessage] =
    (1...30).map { ChatMessage(text: "message \($0)", isUser: Bool.random()) }
    @State private var messageText: String = ""
    
    var body: some View {
        VStack{
            VStack(alignment: .center) {
                Text("Conversation")
                    .font(.title2)
                    .bold()
                
                ScrollView {
                    VStack(alignment: .leading, spacing: 12) {
                        ForEach(messages) { item in
                            HStack {
                                if item.isUser {
                                    Spacer()
                                    Text(item.text)
                                        .padding()
                                        .background(Color.blue.opacity(0.8))
                                        .cornerRadius(10)
                                } else {
                                    Text(item.text)
                                        .padding()
                                        .background(Color.green.opacity(0.9))
                                        .cornerRadius(10)
                                    Spacer()
                                }
                            }
                        }
                    }
                }
                .frame(maxHeight: .infinity)
                .defaultScrollAnchor(.bottom)
            }
            .padding()
            TextField(
                "Ask something...",
                text: $messageText
            ).onSubmit {
                messages.append( ChatMessage(text: $messageText.wrappedValue, isUser: true))
                messageText = ""
            }
            .padding(14)
            .background(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(Color.gray.opacity(0.3), lineWidth: 1)
            )
            .font(.system(size: 16))
            .textInputAutocapitalization(.never)
            .disableAutocorrection(true).padding(.horizontal)
            
        }.frame(maxHeight: .infinity,alignment: .bottom)
    }
}

#Preview {
    ChatView()
}
