import {Component, OnInit, Input} from '@angular/core';
import {FileSharingService} from '../file-sharing.service';

@Component({
  selector: 'app-file-list',
  templateUrl: './file-list.component.html',
  styleUrls: ['./file-list.component.css']
})
export class FileListComponent implements OnInit {
  @Input() directory: string;
  files: File[] = [];

  constructor(private fileSharingService: FileSharingService) {}

  ngOnInit() {
    this.fileSharingService
      .getFiles(this.directory)
      .then(files => {
        console.log("Files!", files);
        this.files = files;
      })
  }
}
