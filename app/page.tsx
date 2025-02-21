import { Button } from "@/components/ui/button"

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center">
      <div className="flex flex-col items-center gap-6 text-center">
        <div className="flex flex-col gap-3">
          <p className="text-primary-950 text-base font-medium">Coming soon</p>
          <h1 className="font-semibold text-white">Framer University</h1>
          <p className="text-primary-950 max-w-[308px] text-base font-medium">
            The world&apos;s first learning platform dedicated to teaching Framer in a fun & efficient way.
          </p>
        </div>
        <div className="flex gap-2.5">
          <Button intent="secondary" size="sm" href="https://framer.university/waitlist">
            Join Waitlist
          </Button>
          <Button intent="primary" size="sm" href="https://framer.university">
            Learn More
          </Button>
        </div>
      </div>
    </div>
  )
}
