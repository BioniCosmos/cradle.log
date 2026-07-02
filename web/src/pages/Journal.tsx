import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from '@/components/ui/card'
import type { Disease } from '@/lib/api'
import { useLoaderData } from 'react-router'

export default function Journal() {
  const diseases = useLoaderData<Disease[]>()

  return (
    <div>
      {diseases.length === 0 ? (
        <p className="text-muted-foreground">No records yet.</p>
      ) : (
        <>
          {diseases.map((disease) => (
            <Card>
              <CardHeader>
                <CardDescription>{disease.creation_time}</CardDescription>
              </CardHeader>
              <CardContent>
                <div>
                  <p>Diseases</p>
                  <div>
                    {disease.diseases.map((disease) => (
                      <Badge>{disease}</Badge>
                    ))}
                  </div>
                </div>
                <div>
                  <p>Symptoms</p>
                  <div>
                    {disease.symptoms.map((symptom) => (
                      <Badge>{symptom}</Badge>
                    ))}
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </>
      )}
    </div>
  )
}
