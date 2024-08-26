import json
from dataclasses import dataclass, asdict
from typing import List, Dict

@dataclass
class Task:
    id: int
    title: str
    completed: bool

class TaskManager:
    def __init__(self):
        self.tasks: List[Task] = []

    def add_task(self, title: str) -> None:
        task_id = len(self.tasks) + 1
        self.tasks.append(Task(id=task_id, title=title, completed=False))

    def complete_task(self, task_id: int) -> None:
        for task in self.tasks:
            if task.id == task_id:
                task.completed = True
                break

    def get_incomplete_tasks(self) -> List[Dict]:
        return [asdict(task) for task in self.tasks if not task.completed]

    def to_json(self) -> str:
        return json.dumps([asdict(task) for task in self.tasks], indent=2)

if __name__ == "__main__":
    manager = TaskManager()
    manager.add_task("Buy groceries")
    manager.add_task("Walk the dog")
    manager.add_task("Pay bills")
    
    manager.complete_task(2)
    
    print("Incomplete tasks:", manager.get_incomplete_tasks())
    print("\nAll tasks:")
    print(manager.to_json())