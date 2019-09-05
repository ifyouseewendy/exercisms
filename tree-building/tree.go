package tree

import (
	"fmt"
	"sort"
)

type Record struct {
	ID     int
	Parent int
}

type Node struct {
	ID       int
	Children []*Node
}

func Build(records []Record) (*Node, error) {
	if len(records) == 0 {
		return nil, nil
	}

	// Pre-sort to keep topology order
	sort.Slice(records, func(i, j int) bool { return records[i].ID < records[j].ID })

	rootID := -1
	recordMap := make(map[int][]int)

	for _, record := range records {
		if record.ID < record.Parent || record.ID >= len(records) || record.Parent < 0 {
			return nil, fmt.Errorf("Outrange error on record: %+v", record)
		} else if record.ID == record.Parent {
			if rootID != -1 {
				return nil, fmt.Errorf("Multiple roots on record: %+v", record)
			}
			rootID = record.ID
		} else {
			recordMap[record.Parent] = append(recordMap[record.Parent], record.ID)
		}
	}

	fmt.Printf("Debug for recordMap: %+v\n", recordMap)

	rootNode := &Node{ID: rootID}
	visited := make(map[int]bool)
	rootNode, err := assembleTree(rootNode, &recordMap, &visited)

	if err != nil {
		return nil, err
	}
	return rootNode, nil
}

func assembleTree(currentNode *Node, recordMap *map[int][]int, visited *map[int]bool) (*Node, error) {
	if (*visited)[currentNode.ID] {
		return nil, fmt.Errorf("Cycle detected: %+v", *recordMap)
	}
	(*visited)[currentNode.ID] = true

	for _, childID := range (*recordMap)[currentNode.ID] {
		newNode := &Node{ID: childID}
		tree, err := assembleTree(newNode, recordMap, visited)

		if err != nil {
			return nil, err
		}

		if currentNode.Children == nil {
			currentNode.Children = make([]*Node, 0)
		}
		currentNode.Children = append(currentNode.Children, tree)
	}

	return currentNode, nil
}
