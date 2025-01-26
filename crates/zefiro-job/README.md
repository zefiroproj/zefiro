# zefiro-job

## Message request structure

```json5
{
    "id": "123",
    "image": "vidjil:latest",
    "min_resources": {
        "cpus": 1,
        "ram": 1024,
        "disk": 1024
    },
    "max_resources": {
        "cpus": 1,
        "ram": 1024,
        "disk": 1024
    },
    "timelimit": 120,
    "args": [
        "--in-fastq=/inputs/in_R12.fastq.gz",
        "--out-fasta=/outputs/output.fasta.gz",
        "--vdj-ref=/inputs/vidjil.germline.only_human.tar.gz"
    ],
    "priority": "medium"
}
```