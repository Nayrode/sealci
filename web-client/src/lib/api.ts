import { Action, Pipeline } from '@/types'

// Données mockées pour les pipelines
const mockPipelines: Pipeline[] = [
  {
    id: 1,
    repository_url: 'https://github.com/vercel/next.js',
    name: 'Build & Deploy Next.js',
    actions: [
      {
        id: 1,
        pipeline_id: 1,
        name: 'Checkout Code',
        container_uri: 'ubuntu:latest',
        commands: ['git clone https://github.com/vercel/next.js', 'cd next.js'],
        type: 'checkout',
        status: 'ACTION_STATUS_COMPLETED',
        logs: [
          "Cloning into 'next.js'...",
          'remote: Enumerating objects: 123456, done.',
          'remote: Counting objects: 100% (123456/123456), done.',
          'Receiving objects: 100% (123456/123456), 45.67 MiB | 12.34 MiB/s, done.',
          'Resolving deltas: 100% (98765/98765), done.',
        ],
      },
      {
        id: 2,
        pipeline_id: 1,
        name: 'Install Dependencies',
        container_uri: 'node:18',
        commands: ['npm ci', 'npm run build'],
        type: 'build',
        status: 'ACTION_STATUS_COMPLETED',
        logs: [
          'npm WARN using --force',
          'npm WARN using --force',
          'added 1234 packages in 45s',
          '> next.js@13.0.0 build',
          '> next build',
          'info  - Checking validity of types...',
          'info  - Creating an optimized production build...',
          'info  - Compiled successfully',
        ],
      },
      {
        id: 3,
        pipeline_id: 1,
        name: 'Run Tests',
        container_uri: 'node:18',
        commands: ['npm test', 'npm run test:e2e'],
        type: 'test',
        status: 'ACTION_STATUS_RUNNING',
        logs: ['PASS src/components/Button.test.tsx', 'PASS src/components/Header.test.tsx', 'PASS src/utils/helpers.test.ts', 'Running e2e tests...', 'Starting test server on port 3000...'],
      },
      {
        id: 4,
        pipeline_id: 1,
        name: 'Deploy to Production',
        container_uri: 'vercel/cli',
        commands: ['vercel --prod'],
        type: 'deploy',
        status: 'ACTION_STATUS_PENDING',
      },
    ],
  },
  {
    id: 2,
    repository_url: 'https://github.com/facebook/react',
    name: 'React CI Pipeline',
    actions: [
      {
        id: 5,
        pipeline_id: 2,
        name: 'Checkout Code',
        container_uri: 'ubuntu:latest',
        commands: ['git clone https://github.com/facebook/react'],
        type: 'checkout',
        status: 'ACTION_STATUS_COMPLETED',
        logs: ['Successfully cloned repository'],
      },
      {
        id: 6,
        pipeline_id: 2,
        name: 'Build React',
        container_uri: 'node:16',
        commands: ['yarn install', 'yarn build'],
        type: 'build',
        status: 'ACTION_STATUS_ERROR',
        logs: [
          'yarn install v1.22.19',
          '[1/4] Resolving packages...',
          '[2/4] Fetching packages...',
          '[3/4] Linking dependencies...',
          '[4/4] Building fresh packages...',
          'Done in 67.89s.',
          'yarn run v1.22.19',
          '$ rollup -c',
          'Error: Build failed with 1 error:',
          'src/React.js:45:23: ERROR: Could not resolve "./ReactVersion"',
          'error Command failed with exit code 1.',
        ],
      },
    ],
  },
  {
    id: 3,
    repository_url: 'https://github.com/microsoft/vscode',
    name: 'VSCode Build Pipeline',
    actions: [
      {
        id: 7,
        pipeline_id: 3,
        name: 'Checkout',
        container_uri: 'ubuntu:latest',
        commands: ['git clone https://github.com/microsoft/vscode'],
        type: 'checkout',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 8,
        pipeline_id: 3,
        name: 'Install & Build',
        container_uri: 'node:16',
        commands: ['npm install', 'npm run compile'],
        type: 'build',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 9,
        pipeline_id: 3,
        name: 'Unit Tests',
        container_uri: 'node:16',
        commands: ['npm test'],
        type: 'test',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 10,
        pipeline_id: 3,
        name: 'Integration Tests',
        container_uri: 'node:16',
        commands: ['npm run test:integration'],
        type: 'test',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 11,
        pipeline_id: 3,
        name: 'Package Application',
        container_uri: 'node:16',
        commands: ['npm run package'],
        type: 'package',
        status: 'ACTION_STATUS_COMPLETED',
      },
    ],
  },
  {
    id: 4,
    repository_url: 'https://github.com/tailwindlabs/tailwindcss',
    name: 'Tailwind CSS Pipeline',
    actions: [
      {
        id: 12,
        pipeline_id: 4,
        name: 'Setup Environment',
        container_uri: 'node:18',
        commands: ['git clone https://github.com/tailwindlabs/tailwindcss', 'cd tailwindcss'],
        type: 'setup',
        status: 'ACTION_STATUS_SCHEDULED',
      },
    ],
  },
  {
    id: 5,
    repository_url: 'https://github.com/vuejs/vue',
    name: 'Vue.js Development Build',
    actions: [
      {
        id: 13,
        pipeline_id: 5,
        name: 'Checkout Repository',
        container_uri: 'alpine/git',
        commands: ['git clone --depth=1 https://github.com/vuejs/vue'],
        type: 'checkout',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 14,
        pipeline_id: 5,
        name: 'Install Dependencies',
        container_uri: 'node:16-alpine',
        commands: ['npm ci --only=production'],
        type: 'install',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 15,
        pipeline_id: 5,
        name: 'Lint Code',
        container_uri: 'node:16-alpine',
        commands: ['npm run lint'],
        type: 'lint',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 16,
        pipeline_id: 5,
        name: 'Build Distribution',
        container_uri: 'node:16-alpine',
        commands: ['npm run build'],
        type: 'build',
        status: 'ACTION_STATUS_RUNNING',
        logs: ['Building for production...', 'Compiling templates...', 'Optimizing assets...', 'Progress: 67% completed'],
      },
    ],
  },
  {
    id: 6,
    repository_url: 'https://github.com/angular/angular',
    name: 'Angular Framework CI',
    actions: [
      {
        id: 17,
        pipeline_id: 6,
        name: 'Environment Setup',
        container_uri: 'node:18',
        commands: ['git clone https://github.com/angular/angular', 'cd angular', 'npm install -g @angular/cli'],
        type: 'setup',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 18,
        pipeline_id: 6,
        name: 'Install Dependencies',
        container_uri: 'node:18',
        commands: ['npm install'],
        type: 'install',
        status: 'ACTION_STATUS_COMPLETED',
      },
      {
        id: 19,
        pipeline_id: 6,
        name: 'Build Angular',
        container_uri: 'node:18',
        commands: ['ng build --prod'],
        type: 'build',
        status: 'ACTION_STATUS_ERROR',
        logs: [
          'Building Angular application...',
          'Analyzing bundle size...',
          'ERROR in src/app/app.component.ts(23,5):',
          "TS2322: Type 'string' is not assignable to type 'number'.",
          'Build failed with 1 error.',
        ],
      },
    ],
  },
]

export const fetchPipelines = async ({
  verbose,
}: {
  verbose?: boolean
} = {}): Promise<Pipeline[]> => {
  // Simuler un délai d'API
  await new Promise((resolve) => setTimeout(resolve, 800))

  if (verbose) {
    return mockPipelines
  } else {
    // Retourner les pipelines sans les logs pour la version non-verbose
    return mockPipelines.map((pipeline) => ({
      ...pipeline,
      actions: pipeline.actions.map((action: Action) => ({
        ...action,
        logs: undefined,
      })),
    }))
  }
}

export const fetchPipeline = async ({ verbose, id }: { verbose: boolean; id: string }): Promise<Pipeline> => {
  // Simuler un délai d'API
  await new Promise((resolve) => setTimeout(resolve, 600))

  const pipeline = mockPipelines.find((p) => p.id === Number.parseInt(id))

  if (!pipeline) {
    throw new Error(`Pipeline with id ${id} not found`)
  }

  if (verbose) {
    return pipeline
  } else {
    return {
      ...pipeline,
      actions: pipeline.actions.map((action: Action) => ({
        ...action,
        logs: undefined,
      })),
    }
  }
}

/////////////////////////////////////////////////////////////////////////
// Uncomment the following code to fetch pipelines from the actual API //
/////////////////////////////////////////////////////////////////////////

// const fetchPipelines = async ({
//   verbose,
// }: {
//   verbose?: boolean
// } = {}): Promise<Pipeline[]> => {
//   const endpoint = verbose ? '/pipeline?verbose=true' : '/pipeline?verbose=false'
//   const json = await ky.get(import.meta.env.VITE_CONTROLLER_ENDPOINT + endpoint).json<Pipeline[]>()

//   return json
// }

// const fetchPipeline = async ({ verbose, id }: { verbose: boolean; id: string }): Promise<Pipeline> => {
//   const endpoint = verbose ? `/pipeline/${id}?verbose=true` : `/pipeline/${id}?verbose=false`
//   const json = await ky.get(import.meta.env.VITE_CONTROLLER_ENDPOINT + endpoint).json<Pipeline>()

//   return json
// }
