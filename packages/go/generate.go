//go:build !kreuzcrawl_dev
// +build !kreuzcrawl_dev

//go:generate go run github.com/kreuzberg-dev/kreuzcrawl/packages/go/cmd/install@latest

// Package kreuzcrawl provides a web crawler library for Go.
//
// The go:generate directive above downloads the FFI library for your platform
// and generates the CGO flags needed to build. Run it once after installing:
//
//	go generate github.com/kreuzberg-dev/kreuzcrawl/packages/go
//
// This eliminates the need to manually set CGO_CFLAGS and CGO_LDFLAGS environment variables.
package kreuzcrawl
