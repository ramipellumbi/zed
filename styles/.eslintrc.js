module.exports = {
    env: {
        node: true,
    },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:import/typescript",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
    },
    plugins: ["@typescript-eslint", "import"],
    globals: {
        module: true,
    },
    settings: {
        "import/parsers": {
            "@typescript-eslint/parser": [".ts"],
        },
        "import/resolver": {
            typescript: true,
            node: true,
        },
        "import/extensions": [".ts"],
    },
    rules: {
        "linebreak-style": ["error", "unix"],
        semi: ["error", "never"],
        "@typescript-eslint/naming-convention": [
            "warn",
            {
                selector: ["property", "variableLike", "memberLike", "method"],
                format: ["snake_case"],
            },
            {
                selector: ["typeLike"],
                format: ["PascalCase"],
            },
        ],
        "import/no-restricted-paths": [
            "error",
            {
                zones: [
                    {
                        target: [
                            "./src/component/*",
                            "./src/element/*",
                            "./src/styleTree/*",
                            "./src/system/*",
                            "./src/theme/*",
                            "./src/themes/*",
                            "./src/utils/*",
                        ],
                        from: [
                            "./src/types/styleTree.ts",
                            "./src/types/element.ts",
                            "./src/types/property.ts",
                        ],
                        message: "Import from `@types` instead",
                    },
                ],
            },
        ],
    },
}
