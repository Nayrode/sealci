export interface Monitor {
  repo_owner: string
  repo_name: string
  repo_url: string
  events: string[]
}

export interface CreateMonitor {
  repo_owner: string
  repo_name: string
  event: string
  file: File
  github_token: string
}

export interface Pipeline {
  id: number
  repository_url: string
  name: string
  actions: Action[]
}

export interface CreatePipeline {
  repo_url: string
  body: File
}

export interface Action {
  id: number
  pipeline_id: number
  name: string
  container_uri: string
  commands: string[]
  type: string
  status: string
  logs?: string[]
}

export type PipelineStatus = 'ACTION_STATUS_PENDING' | 'ACTION_STATUS_SCHEDULED' | 'ACTION_STATUS_RUNNING' | 'ACTION_STATUS_COMPLETED' | 'ACTION_STATUS_ERROR'
