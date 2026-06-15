import { cn } from '@/lib/utils'
import { Link } from 'react-router'

export default function Logo({ className }: { className?: string }) {
  return (
    <Link
      to="/"
      className={cn(
        'flex items-center gap-2.5 font-heading text-xl tracking-tight',
        className,
      )}
    >
      <span
        className="size-5 rounded-full bg-[radial-gradient(circle_at_35%_35%,oklch(0.72_0.105_46),oklch(0.625_0.135_42)_70%)]"
        aria-hidden="true"
      />
      <div>Cradle.log</div>
    </Link>
  )
}
