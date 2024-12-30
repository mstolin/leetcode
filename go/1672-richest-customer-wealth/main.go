package main

import (
	"math"
)

func GetHighestWealth(data [][]uint) uint {
	//var data = [3][3]uint{{2, 8, 7}, {7, 1, 3}, {1, 9, 5}}
	var highest uint

	for _, customer := range data {
		var wealth uint
		for _, account := range customer {
			wealth += account
		}
		highest = uint(math.Max(float64(highest), float64(wealth)))
	}

	return highest
}
