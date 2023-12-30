//
//  main.swift
//  day08
//
//  Created by Patryk Pekala on 30.12.23.
//

import Foundation

let input = try String(contentsOfFile: "input")
let lines = input.split(separator: "\n")

let directions = String(lines[0])
let directionArray = Array(directions)

let nodes = lines[1...].map { Node(nodeStr: String($0)) }

let nodeMap = Dictionary(uniqueKeysWithValues: nodes.map { ($0.name, $0) })

// Part 1

var currNode = nodeMap["AAA"]!
var steps = 0

while (currNode.name != "ZZZ") {
    let direction = directionArray[steps % directions.count]
    
    if direction == "L" {
        currNode = nodeMap[currNode.leftDirection]!
    } else if direction == "R" {
        currNode = nodeMap[currNode.rightDirection]!
    }
    
    steps += 1
}

print("Number of steps from AAA to ZZZ: \(steps)")

// Part 2

func findLoopLength(node: Node, directions: [Character], nodeMap: Dictionary<String, Node>) -> Int {
    var currNode = node
    var steps = 0
    var directionIndex = 0
    
    while (!currNode.name.hasSuffix("Z")) {
        if directionIndex == directions.count {
            directionIndex = 0
        }
        
        currNode = currNode.pickNextNode(left: directions[directionIndex] == "L", nodeMap: nodeMap)
        steps += 1
        directionIndex += 1
    }
    
    return steps
}

let startNodes = nodes.filter { node in node.name.hasSuffix("A") }
let loopLengths = startNodes.map { findLoopLength(node: $0, directions: directionArray, nodeMap: nodeMap) }

let stepsFromAnodesToZnodes = loopLengths.reduce(1) { lcm($0, $1) }

print("Steps from nodes that end with A so that all end with Z: \(stepsFromAnodesToZnodes)")
