import {Component, OnInit, Input} from '@angular/core';
import {FileSharingService} from '../file-sharing.service';
import { ActivatedRoute, Params } from '@angular/router';

import 'rxjs/add/operator/switchMap';


@Component({
  selector: 'app-file-list',
  templateUrl: './file-list.component.html',
  styleUrls: ['./file-list.component.css']
})
export class FileListComponent implements OnInit {
  directory: string;
  private sub: any;
  files: File[] = [];

  constructor(private fileSharingService: FileSharingService, private route: ActivatedRoute) {}

  ngOnInit() {
    this.route.params
      .switchMap((params: Params) => {
        this.directory = params['directory'];
        return this.fileSharingService.getFiles(params['directory']);
      }).
    subscribe((files: File[]) => {
      console.log("Files are", files);
      this.files = files;
    })
  }
}
