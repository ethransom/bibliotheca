package main

import (
	"errors"
	"fmt"
	"os"
	"path"
	"path/filepath"
)

const obvsDir = ".obvs"

func cmdInit(args []string) error {
	rootPath := "."
	if len(args) > 0 {
		rootPath = args[0]
	}
	obvsPath, err := filepath.Abs(path.Join(rootPath, obvsDir))
	if err != nil {
		return err
	}
	for _, dir := range [2]string{"objects", "refs"} {
		err := os.MkdirAll(path.Join(obvsPath, dir), os.ModePerm)
		if err != nil {
			return err
		}
	}
	fmt.Printf("Initialized empty Obvs repository in %s\n", obvsPath)
	return nil
}

func cmdHelp(args []string) error {
	println("help: read the source")
	return nil
}

func command(args []string) error {
	if len(args) < 1 {
		return errors.New("must give subcommand")
	}

	switch args[0] {
	case "init":
		return cmdInit(args[1:])
	case "help":
		return cmdHelp(args[1:])
	default:
		return fmt.Errorf("no subcommand '%s'", args[0])
	}
}

func main() {
	if err := command(os.Args[1:]); err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}
