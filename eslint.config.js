import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';

export default [
  {
    ignores: [
      'node_modules/**',
      '.svelte-kit/**',
      'dist/**',
      'build/**',
      'coverage/**',
      'target/**'
    ]
  },
  js.configs.recommended,
  ...svelte.configs['flat/recommended']
];
