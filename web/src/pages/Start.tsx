import { Button } from '@/components/ui/button'
import Wave from '@/components/Wave'
import { SendHorizonal } from 'lucide-react'
import './Start.css'

export default function Start() {
  const prompt = 'How do you feeling today?'

  return (
    <div className="flex h-[calc(100dvh-3.25rem)] flex-col">
      <div className="flex flex-1 flex-col items-center justify-center gap-25 pb-8">
        <div aria-hidden="true">
          <div className="absolute size-26 animate-[ripple] rounded-full border border-primary/40 ease-out animation-duration-[3s] repeat-infinite" />
          <div className="absolute size-26 animate-[ripple] rounded-full border border-primary/30 delay-[1s] ease-out animation-duration-[3s] repeat-infinite" />
          <div className="absolute size-26 animate-[ripple] rounded-full border border-primary/20 delay-[2s] ease-out animation-duration-[3s] repeat-infinite" />
          <div
            className="size-26 animate-[orb-pulse] rounded-full ease-in-out animation-duration-[3s] repeat-infinite"
            style={{
              background:
                'radial-gradient(circle at 35% 35%, oklch(0.72 0.105 46), oklch(0.553 0.195 38) 70%)',
            }}
          />
        </div>
        <p className="max-w-4/5 text-center font-heading text-3xl font-light text-balance">
          {prompt}
        </p>
      </div>
      <div className="fixed bottom-0 left-0 -z-10 w-full" aria-hidden="true">
        <Wave
          className="absolute bottom-0 left-0 h-55 w-[200%] animate-[wave] opacity-18 ease-linear animation-duration-[13s] repeat-infinite"
          fill="oklch(0.625 0.135 42)"
        />
        <Wave
          className="absolute bottom-0 left-0 h-65 w-[200%] animate-[wave] opacity-13 ease-linear animation-duration-[19s] direction-reverse repeat-infinite"
          fill="oklch(0.72 0.105 46)"
        />
        <Wave
          className="absolute bottom-0 left-0 h-75 w-[200%] animate-[wave] opacity-10 delay-[-5s] ease-linear animation-duration-[27s] repeat-infinite"
          fill="oklch(0.625 0.135 42)"
        />
      </div>
      <div className="mx-auto mb-6 flex w-3/5 items-center gap-2 rounded-xl bg-card px-3 py-1.5 shadow">
        <textarea
          name="journal"
          className="field-sizing-content max-h-[60dvh] grow resize-none outline-none"
        />
        <Button size="icon-lg" className="self-end">
          <SendHorizonal className="size-5" />
        </Button>
      </div>
    </div>
  )
}
