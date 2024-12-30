package main

import "testing"

func TestGetHighestWealth(t *testing.T) {
	var data = [][]uint{{2, 8, 7}, {7, 1, 3}, {1, 9, 5}}
	var expected uint = 17
	var highest = GetHighestWealth(data)
	if highest != expected {
		t.Errorf("expected: %d, got: %d", expected, highest)
	}
}
