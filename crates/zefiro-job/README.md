# zefiro-job

## Message request structure

```json5
{
    "job_id": "123",
    "tool": {
        "image": "vidjil:latest",
        "min_ram": 1024,
        "min_cpus": 1,
        "min_disk": 1024,
        "max_ram": 1024,
        "max_cpus": 1,
        "max_disk": 1024,
        "timelimit": 120,
        "args": [
            "--in-fastq=/inputs/in_R12.fastq.gz",
            "--out-fasta=/outputs/output.fasta.gz",
            "--vdj-ref=/inputs/vidjil.germline.only_human.tar.gz"
        ]
    },
    "priority": "normal"
}
```