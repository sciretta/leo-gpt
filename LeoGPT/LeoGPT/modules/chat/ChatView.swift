//
//  ChatView.swift
//  LeoGPT
//
//  Created by Leonardo on 23/11/25.
//

import SwiftUI

struct ChatView: View {
    let chatId: String
    @State private var messages: [Message] = []
    @State private var messageText: String = ""
    @EnvironmentObject var appState: AppState
    
    var body: some View {
        VStack{
            VStack(alignment: .center) {
                
                ScrollView {
                    VStack(alignment: .leading, spacing: 12) {
                        ForEach(messages, id: \.self) { item in
                            HStack {
                                if item.isUser {
                                    Spacer()
                                    Text(item.content)
                                        .padding()
                                        .background(Color.blue.opacity(0.8))
                                        .cornerRadius(10)
                                } else {
                                    Text(item.content)
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
                let currentMessage = messageText
                if chatId.isEmpty {
                    Task {
                        do {
                            let res = try await createChat(userId: appState.userId, chatName: String(currentMessage.prefix(20)))
                            
                            print("Chat created", res)
                            
                            if res {
                                print("create message logic here")
                            }
                            
                        } catch {
                            print("Error", error)
                        }
                    }
                }else {
                    Task {
                        do {
                            print("create message logic here")
                            
                            
                        } catch {
                            print("Error", error)
                        }
                    }
                }
                
                Task {
                    do {
                        if !chatId.isEmpty {
                            let newMessages = try await fetchMessages(chatId:chatId)
                            messages = newMessages
                        }
                    } catch {
                        print("Error", error)
                    }
                }
                
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
            
        }.onAppear{
            Task {
                do {
                    if !chatId.isEmpty {
                        let newMessages = try await fetchMessages(chatId:chatId)
                        messages = newMessages
                    }
                } catch {
                    print("Error", error)
                }
            }
        }
        .frame(maxHeight: .infinity,alignment: .bottom)
    }
}

#Preview {
    ChatView(chatId: "69dbfef957eb0152781f9ffc").environmentObject(AppState())
}
