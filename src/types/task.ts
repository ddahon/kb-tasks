export enum TaskStatus {
  Completed = "Completed",
  Deleted = "Deleted",
  Created = "Created",
}

export interface Task {
  id: number;
  title: string;
  desc: string;
  status: TaskStatus;
}
