//
//  Node.swift
//  day08
//
//  Created by Patryk Pekala on 30.12.23.
//

import Foundation

class Node {
    let name: String
    let leftDirection: String
    let rightDirection: String
    
    init(nodeStr: String) {
        let nameAndDirections = nodeStr.split(separator: "=")
        
        name = String(nameAndDirections[0]).trimmingCharacters(in: .whitespaces)
        
        let directions = nameAndDirections[1].split(separator: ",")
            .map { $0
                    .trimmingCharacters(in: .whitespaces)
                    .trimmingCharacters(in: CharacterSet(["(", ")"]))
            }
        
        leftDirection = directions[0]
        rightDirection = directions[1]
    }
    
    func pickNextNode(left: Bool, nodeMap: Dictionary<String, Node>) -> Node {
        nodeMap[left ? leftDirection : rightDirection]!
    }
}
