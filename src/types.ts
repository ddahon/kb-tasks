enum TaskStatus {
  Completed,
  Deleted,
  Created,
}

interface Task {
  id: number;
  title: string;
  desc: string;
  status: TaskStatus;
}
