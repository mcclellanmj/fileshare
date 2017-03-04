import { Component, OnInit, Input } from '@angular/core';
import { Router } from '@angular/router';
import { URLSearchParams } from '@angular/http';

@Component({
  selector: 'app-file-list-item',
  templateUrl: './file-list-item.component.html',
  styleUrls: ['./file-list-item.component.css']
})
export class FileListItemComponent implements OnInit {
  @Input() file: any;
  link: any;
  iconClass: string;
  containerClass: string;

  constructor(private router: Router) {
  }

  ngOnInit() {
    if(this.file.isFolder) {
      this.link = ["/filelist", this.file.fullPath];
      this.iconClass = "fa-folder";
      this.containerClass = "folder-item";
    } else {
      var urlSearchParams: URLSearchParams = new URLSearchParams();
      urlSearchParams.set("filename", this.file.fullPath);

      this.link = `/download?${urlSearchParams.toString()}`;
      this.iconClass = "fa-file";
      this.containerClass = "file-item";
    }
  }
}
