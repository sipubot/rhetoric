$PSDefaultParameterValues = @{ '*:Encoding' = 'utf8' }

$ntesturl = "https://datalab.naver.com/keyword/realtimeList.naver?where=main";
$dtesturl = "https://www.daum.net";
$nallfile = "C:\Sipu\VS\Rust\powershell\naver_all.txt";
$nlv1file = "C:\Sipu\VS\Rust\powershell\naver_lv1.txt";
$nlv2file = "C:\Sipu\VS\Rust\powershell\naver_lv2.txt";
$nlv3file = "C:\Sipu\VS\Rust\powershell\naver_lv3.txt";
$nlv4file = "C:\Sipu\VS\Rust\powershell\naver_lv4.txt";
$nlv5file = "C:\Sipu\VS\Rust\powershell\naver_lv5.txt";
$dfile = "C:\Sipu\VS\Rust\powershell\daum_all.txt";

###naver
$nres = Invoke-WebRequest $ntesturl;
$listitem = $nres.AllElements | Where-Object {
    $_.class -match "rank_inner"
};

$all = $listitem | Where-Object { $_.'data-age' -match "all"};
$lv1 = $listitem | Where-Object { $_.'data-age' -match "10s"};
$lv2 = $listitem | Where-Object { $_.'data-age' -match "20s"};
$lv3 = $listitem | Where-Object { $_.'data-age' -match "30s"};
$lv4 = $listitem | Where-Object { $_.'data-age' -match "40s"};
$lv5 = $listitem | Where-Object { $_.'data-age' -match "50s"};

$all = $all.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};
$lv1 = $lv1.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};
$lv2 = $lv2.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};
$lv3 = $lv3.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};
$lv4 = $lv4.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};
$lv5 = $lv5.innerHTML -split '"title">' | ForEach-Object { ($_ -split '</span>')[0]} | Where-Object { $_ -notmatch '<span'};

### daum
$dres = Invoke-WebRequest $dtesturl;
$daumList = $dres.AllElements | Where-Object {
    $_.class -match "txt_issue";
} | ForEach-Object { $_.innerText } | Select -Unique;

$all | Out-File $nallfile;
$lv1 | Out-File $nlv1file;
$lv2 | Out-File $nlv2file;
$lv3 | Out-File $nlv3file;
$lv4 | Out-File $nlv4file;
$lv5 | Out-File $nlv5file;
$daumList | Out-File $dfile;


$MainPoint;
$SubPoint;

C:\Sipu\VS\Rust\powershell\demo.exe test.txt re.txt naver_all.txt;

$re = Get-Content -Path re.txt;

$re | ForEach-Object {
    $reid = $_.split(":")[0];
    $rect =  $_.split(":")[1];

    if ($SubPoint.ContainsKey($reid)) {
        $SubPoint[$reid] = $SubPoint[$reid] + $rect;
    } else {
        $SubPoint[$reid] = $rect;
    }
}
##loop scrap
$SiteA = $SubPoint;

## compare Sub Make Main

## Parse to Json
