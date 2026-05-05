package commands

import (
	"errors"
	"math"
	"strconv"
)

func RequestFingerprint(command Command) (string, error) {
	if command.Operator == "" || math.IsNaN(command.Value) || math.IsInf(command.Value, 0) {
		return "", errors.New("invalid idempotency input")
	}
	return string(command.Kind) + "|" + command.Operator + "|" + strconv.FormatFloat(command.Value, 'g', -1, 64), nil
}
