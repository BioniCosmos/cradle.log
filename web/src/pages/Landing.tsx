import { buttonVariants } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { ArrowRight } from 'lucide-react'
import { Link } from 'react-router'

export function Landing() {
  return (
    <div className="min-h-dvh antialiased">
      <div className="mx-auto max-w-300 px-5.5 md:px-10">
        <Link
          to="/"
          className="flex items-center gap-2.5 pt-7 font-heading text-2xl tracking-tight"
        >
          <span
            className="size-5 rounded-full bg-[radial-gradient(circle_at_35%_35%,oklch(0.72_0.105_46),oklch(0.625_0.135_42)_70%)]"
            aria-hidden="true"
          />
          <div>Cradle.log</div>
        </Link>
        <div className="pt-20">
          <div className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
            ⸺ A personal log for your body, spirit and soul
          </div>
          <div className="mt-12 w-fit bg-[linear-gradient(135deg,oklch(0.32_0.20_28)_0%,oklch(0.58_0.22_40)_50%,oklch(0.80_0.16_62)_100%)] bg-clip-text font-heading text-[clamp(50px,8vw,84px)]/[1.2] font-medium text-transparent">
            Track
            <br />
            Accompany
            <br />
            Guard
          </div>
          <p className="mt-13 max-w-[50ch] text-lg text-secondary-foreground">
            Write how you feel — symptoms, sleep, mood, whatever's on your mind.
            Cradle.log reads it, finds the patterns, and keeps a quiet record
            you can look back along.
          </p>
          <Link
            to="/journal"
            className={cn(
              buttonVariants({ size: 'lg' }),
              'mt-8 mb-14 rounded-full px-4 py-6 text-base',
            )}
          >
            Start your log
            <ArrowRight data-icon="inline-end" />
          </Link>
        </div>
        <div className="flex flex-col border-y md:flex-row">
          {[
            {
              title: 'Write freely',
              body: 'No forms. No dropdowns. Describe how you feel in plain words — Cradle.log understands the rest.',
            },
            {
              title: 'See patterns',
              body: 'Symptoms, medications, mood, and sleep surface as a quiet timeline. Patterns emerge over weeks, not pop-ups.',
            },
            {
              title: 'Stay private',
              body: 'Your record lives with you. No ads, no data brokers. Export or delete everything, any time.',
            },
          ].map(({ title, body }) => (
            <div
              key={title}
              className="flex flex-1 flex-col gap-4 border-b px-9 py-10 last:border-b-0 md:border-r md:border-b-0 md:last:border-r-0"
            >
              <h3 className="font-heading text-2xl">{title}</h3>
              <p className="text-sm text-muted-foreground">{body}</p>
            </div>
          ))}
        </div>
        <div className="h-8" />
      </div>
    </div>
  )
}
