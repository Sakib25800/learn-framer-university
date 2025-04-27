import { Button } from "@framer-university/ui";

function App() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-black">
      <div className="flex flex-col items-center gap-6 text-center">
        <div className="flex flex-col gap-3">
          <p className="text-primary-950 text-base font-medium">
            Admin Dashboard
          </p>
          <h1 className="font-semibold text-white">Framer University</h1>
          <p className="text-primary-950 max-w-[308px] text-base font-medium">
            Manage and create content
          </p>
        </div>
        <div className="flex gap-2.5">
          <Button intent="secondary" size="sm" href="/courses">
            View Courses
          </Button>
          <Button intent="primary" size="sm" href="/create">
            Create Course
          </Button>
        </div>
      </div>
    </div>
  );
}

export default App;
